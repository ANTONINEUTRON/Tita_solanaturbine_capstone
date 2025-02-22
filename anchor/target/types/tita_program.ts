/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/tita_program.json`.
 */
export type TitaProgram = {
  "address": "8PJFAdH2RJ2v1zdME3HU477yvHf7LRheLWd3xxeSbrsZ",
  "metadata": {
    "name": "titaProgram",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "closeCampaign",
      "discriminator": [
        65,
        49,
        110,
        7,
        63,
        238,
        206,
        77
      ],
      "accounts": [
        {
          "name": "grantCampaign",
          "writable": true
        },
        {
          "name": "grantProvider",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "createCampaign",
      "discriminator": [
        111,
        131,
        187,
        98,
        160,
        193,
        114,
        244
      ],
      "accounts": [
        {
          "name": "grantProvider",
          "writable": true,
          "signer": true
        },
        {
          "name": "grantCampaign",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "campaignId"
              },
              {
                "kind": "account",
                "path": "grantProvider"
              }
            ]
          }
        },
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "grantCampaign"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "tokenMint"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "campaignId",
          "type": "string"
        },
        {
          "name": "totalFunding",
          "type": "u64"
        },
        {
          "name": "deadline",
          "type": {
            "option": "i64"
          }
        },
        {
          "name": "bumps",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createMilestone",
      "discriminator": [
        239,
        58,
        201,
        28,
        40,
        186,
        173,
        48
      ],
      "accounts": [
        {
          "name": "applicant",
          "writable": true,
          "signer": true
        },
        {
          "name": "milestone",
          "writable": true
        },
        {
          "name": "proposal",
          "writable": true
        },
        {
          "name": "grantCampaign",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "milestoneId",
          "type": "u8"
        },
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "proofUri",
          "type": "string"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createProposal",
      "discriminator": [
        132,
        116,
        68,
        174,
        216,
        160,
        198,
        22
      ],
      "accounts": [
        {
          "name": "applicant",
          "writable": true,
          "signer": true
        },
        {
          "name": "proposal",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "grantCampaign"
              },
              {
                "kind": "account",
                "path": "applicant"
              }
            ]
          }
        },
        {
          "name": "grantCampaign"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "setMilestoneStatus",
      "discriminator": [
        67,
        151,
        224,
        85,
        107,
        200,
        42,
        96
      ],
      "accounts": [
        {
          "name": "grantProvider",
          "writable": true,
          "signer": true
        },
        {
          "name": "milestone",
          "writable": true
        },
        {
          "name": "proposal"
        },
        {
          "name": "grantCampaign"
        }
      ],
      "args": [
        {
          "name": "status",
          "type": {
            "defined": {
              "name": "milestoneStatus"
            }
          }
        }
      ]
    },
    {
      "name": "submitProof",
      "discriminator": [
        54,
        241,
        46,
        84,
        4,
        212,
        46,
        94
      ],
      "accounts": [
        {
          "name": "milestone",
          "writable": true
        },
        {
          "name": "proposal"
        },
        {
          "name": "applicant",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "proofUri",
          "type": "string"
        }
      ]
    },
    {
      "name": "updateCampaign",
      "discriminator": [
        235,
        31,
        39,
        49,
        121,
        173,
        19,
        92
      ],
      "accounts": [
        {
          "name": "grantCampaign",
          "writable": true
        },
        {
          "name": "grantProvider",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "totalFunding",
          "type": "u64"
        },
        {
          "name": "isActive",
          "type": "bool"
        }
      ]
    },
    {
      "name": "updateProposalStatus",
      "discriminator": [
        9,
        171,
        178,
        233,
        228,
        50,
        167,
        206
      ],
      "accounts": [
        {
          "name": "proposal",
          "writable": true
        },
        {
          "name": "grantCampaign"
        },
        {
          "name": "grantProvider",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "status",
          "type": {
            "defined": {
              "name": "proposalStatus"
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "grantCampaign",
      "discriminator": [
        131,
        57,
        47,
        16,
        77,
        141,
        235,
        222
      ]
    },
    {
      "name": "milestone",
      "discriminator": [
        38,
        210,
        239,
        177,
        85,
        184,
        10,
        44
      ]
    },
    {
      "name": "proposal",
      "discriminator": [
        26,
        94,
        189,
        187,
        116,
        136,
        53,
        33
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "insufficientFunds",
      "msg": "Insufficient funds for your ask"
    },
    {
      "code": 6001,
      "name": "calculationError",
      "msg": "An error occured while calculating balance"
    },
    {
      "code": 6002,
      "name": "noRemainingFunds",
      "msg": "No remaining funds"
    }
  ],
  "types": [
    {
      "name": "grantCampaign",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalFunding",
            "type": "u64"
          },
          {
            "name": "remainingFunding",
            "type": "u64"
          },
          {
            "name": "isActive",
            "type": "bool"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "updatedAt",
            "type": "i64"
          },
          {
            "name": "deadline",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "grantProvider",
            "type": "pubkey"
          },
          {
            "name": "campaignId",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "milestone",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposal",
            "type": "pubkey"
          },
          {
            "name": "milestoneId",
            "type": "u8"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "updatedAt",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "proofUri",
            "type": "string"
          },
          {
            "name": "status",
            "type": {
              "defined": {
                "name": "milestoneStatus"
              }
            }
          }
        ]
      }
    },
    {
      "name": "milestoneStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "pending"
          },
          {
            "name": "inProgress"
          },
          {
            "name": "completed"
          },
          {
            "name": "approved"
          },
          {
            "name": "rejected"
          }
        ]
      }
    },
    {
      "name": "proposal",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "grantCampaign",
            "type": "pubkey"
          },
          {
            "name": "applicant",
            "type": "pubkey"
          },
          {
            "name": "status",
            "type": {
              "defined": {
                "name": "proposalStatus"
              }
            }
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "updatedAt",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "proposalStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "pending"
          },
          {
            "name": "approved"
          },
          {
            "name": "rejected"
          }
        ]
      }
    }
  ]
};
