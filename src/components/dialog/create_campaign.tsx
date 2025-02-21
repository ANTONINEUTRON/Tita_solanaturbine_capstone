import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "../ui/dialog";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import { useState } from "react";

export default function CreateCampaign() {
    const { connection } = useConnection();
    const { publicKey, sendTransaction } = useWallet();
    const [isLoading, setIsLoading] = useState(false);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!publicKey) return;
        
        setIsLoading(true);
        try {
            const latestBlockhash = await connection.getLatestBlockhash();
            
            const message = new TransactionMessage({
                payerKey: publicKey,
                recentBlockhash: latestBlockhash.blockhash,
                instructions: [], // Add instructions here
            }).compileToV0Message();
            
            const transaction = new VersionedTransaction(message);
            
            const signature = await sendTransaction(transaction, connection);
            await connection.confirmTransaction(signature);
            
        } catch (error) {
            console.error('Error:', error);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div>
            <Dialog>
                <DialogTrigger asChild>
                    <button className='btn btn-primary my-4 text-white'>
                        Create Campaign
                    </button>
                </DialogTrigger>
                <DialogContent className="sm:max-w-[425px]">
                    <form onSubmit={handleSubmit}>
                        <DialogHeader>
                            <DialogTitle>CREATE CAMPAIGN</DialogTitle>
                            <DialogDescription>
                                Create a new grant campaign by providing the required details below.
                            </DialogDescription>
                        </DialogHeader>
                        <div className="flex flex-col space-y-2 my-6">
                            <div className="">
                                <input 
                                    id="title" 
                                    placeholder="Campaign title"
                                    className="w-full p-2 border rounded-md" 
                                />
                            </div>
                            <div className="">
                                <textarea 
                                    id="description"
                                    placeholder="Campaign description" 
                                    className="w-full p-2 border rounded-md min-h-[100px]"
                                />
                            </div>
                            <div className="">
                                <input 
                                    id="deadline"
                                    type="datetime-local"
                                    className="w-full p-2 border rounded-md"
                                />
                            </div>
                        </div>
                        <DialogFooter>
                            <button 
                                type="submit" 
                                className="btn btn-primary mt-4 text-white"
                                disabled={isLoading}
                            >
                                {isLoading ? 'Creating...' : 'Create Campaign'}
                            </button>
                        </DialogFooter>
                    </form>
                </DialogContent>
            </Dialog>
        </div>
    );
}