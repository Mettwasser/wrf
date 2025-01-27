import type { User } from './user';

export interface LobbyAndUser {
    lobby: Lobby;
    user: User;
}

export interface Lobby {
    createdAt: string; // ISO 8601 format for DateTime with TimeZone
    updatedAt: string; // ISO 8601 format for DateTime with TimeZone
    id: number; // Primary key
    expiry: string; // ISO 8601 format for DateTime
    region: keyof typeof REGIONS;
    refinement: RelicRefinement;
    activity: string;
    size: number;
    userId: number;
}

// Enum for Regions
export const REGIONS = {
    AS: 'AS',
    EER: 'EER',
    EU: 'EU',
    NA: 'NA',
    OC: 'OC',
    SA: 'SA',
} as const;
export type Region = (typeof REGIONS)[keyof typeof REGIONS];

// Enum for RelicRefinement
export type RelicRefinement = 'Intact' | 'Exceptional' | 'Flawless' | 'Radiant';

export const mock: LobbyAndUser[] = JSON.parse(`
[
  {
    "user": {
      "id": 187,
      "name": "Your Mom"
    },
    "lobby": {
      "createdAt": "2025-01-27T20:14:53.754949Z",
      "updatedAt": "2025-01-27T20:14:53.754949Z",
      "id": 5,
      "expiry": "2025-01-28T00:14:53.754108",
      "region": "EU",
      "refinement": "Radiant",
      "activity": "Requiem IV",
      "size": 4,
      "userId": 187
    }
  },
  {
    "user": {
      "id": 187,
      "name": "Your Mom"
    },
    "lobby": {
      "createdAt": "2025-01-27T20:14:53.754949Z",
      "updatedAt": "2025-01-27T20:14:53.754949Z",
      "id": 5,
      "expiry": "2025-01-28T00:14:53.754108",
      "region": "EU",
      "refinement": "Radiant",
      "activity": "Requiem IV",
      "size": 4,
      "userId": 187
    }
  },
  {
    "user": {
      "id": 187,
      "name": "Your Mom"
    },
    "lobby": {
      "createdAt": "2025-01-27T20:14:53.754949Z",
      "updatedAt": "2025-01-27T20:14:53.754949Z",
      "id": 5,
      "expiry": "2025-01-28T00:14:53.754108",
      "region": "EU",
      "refinement": "Radiant",
      "activity": "Requiem IV",
      "size": 4,
      "userId": 187
    }
  },
  {
    "user": {
      "id": 187,
      "name": "Your Mom"
    },
    "lobby": {
      "createdAt": "2025-01-27T20:14:53.754949Z",
      "updatedAt": "2025-01-27T20:14:53.754949Z",
      "id": 5,
      "expiry": "2025-01-28T00:14:53.754108",
      "region": "EU",
      "refinement": "Radiant",
      "activity": "Requiem IV",
      "size": 4,
      "userId": 187
    }
  },
  {
    "user": {
      "id": 187,
      "name": "Your Mom"
    },
    "lobby": {
      "createdAt": "2025-01-27T20:14:53.754949Z",
      "updatedAt": "2025-01-27T20:14:53.754949Z",
      "id": 5,
      "expiry": "2025-01-28T00:14:53.754108",
      "region": "EU",
      "refinement": "Radiant",
      "activity": "Requiem IV",
      "size": 4,
      "userId": 187
    }
  },
  {
    "user": {
      "id": 187,
      "name": "Your Mom"
    },
    "lobby": {
      "createdAt": "2025-01-27T20:14:53.754949Z",
      "updatedAt": "2025-01-27T20:14:53.754949Z",
      "id": 5,
      "expiry": "2025-01-28T00:14:53.754108",
      "region": "EU",
      "refinement": "Radiant",
      "activity": "Requiem IV",
      "size": 4,
      "userId": 187
    }
  }
]
`);
