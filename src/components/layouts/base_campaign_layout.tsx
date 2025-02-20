"use client"

import { useState } from 'react'
import { ExistingCampaignsLayout } from './existing_campaigns'
import { NoCampaignsLayout } from './no_campaigns'

export function BaseCampaignsLayout() {
    const [campaigns, setCampaigns] = useState([])
    return (
        <div>
            {
                campaigns.length > 0 ? (
                    <ExistingCampaignsLayout  />
                ) : (
                    <NoCampaignsLayout />
                )
            }
        </div>
    )
}