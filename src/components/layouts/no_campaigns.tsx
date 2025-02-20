"use client"

import Image from 'next/image'

export function NoCampaignsLayout() {
    return (
        <div>
            <section className='w-full flex justify-between h-[86vh]'>
                <div className='flex justify-center items-center'>
                    <div className='md:mx-12 mx-6 text-white'>
                        <div className='text-4xl text-tertiary font-semibold mb-2'>
                            No Campaigns yet
                        </div>
                        <div className='text-lg dark:text-white text-slate-700 font-thin'>
                            Create a campaign to get started
                        </div>
                        <button className='btn btn-primary my-4 text-white'>
                            Create Campaign
                        </button>
                    </div>
                </div>
                <div className='md:flex hidden z-50 justify-center ml-5 items-center animate-pulse'>
                    <Image src="/belp_landing.png" className='w-3/6' width={700} height={700} alt="My Image" />
                </div>
            </section>
        </div>
    )
}