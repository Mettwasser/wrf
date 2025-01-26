export type Lobby = {
    createdAt: string; // ISO 8601 format for DateTime with TimeZone
    updatedAt: string; // ISO 8601 format for DateTime with TimeZone
    id: number; // Primary key
    expiry: string; // ISO 8601 format for DateTime
    region: Region;
    refinement: RelicRefinement;
    activity: string;
    size: number;
    userId: number;
};

// Enum for Regions
export type Region = 'AS' | 'EER' | 'EU' | 'NA' | 'OC' | 'SA';

// Enum for RelicRefinement
export type RelicRefinement = 'Intact' | 'Exceptional' | 'Flawless' | 'Radiant';

export const mock: Lobby[] = JSON.parse(`[
  {
    "createdAt": "2025-01-10T08:00:00Z",
    "updatedAt": "2025-01-12T10:30:00Z",
    "id": 101,
    "expiry": "2025-01-20T12:00:00Z",
    "region": "EU",
    "refinement": "Flawless",
    "activity": "Axi A4",
    "size": 4,
    "userId": 201
  },
  {
    "createdAt": "2025-01-11T09:15:00Z",
    "updatedAt": "2025-01-15T11:45:00Z",
    "id": 102,
    "expiry": "2025-02-01T23:59:59Z",
    "region": "NA",
    "refinement": "Intact",
    "activity": "Axi A4",
    "size": 4,
    "userId": 202
  },
  {
    "createdAt": "2025-01-13T14:45:00Z",
    "updatedAt": "2025-01-14T16:00:00Z",
    "id": 103,
    "expiry": "2025-01-25T18:00:00Z",
    "region": "AS",
    "refinement": "Radiant",
    "activity": "Axi A4",
    "size": 4,
    "userId": 203
  },
  {
    "createdAt": "2025-01-14T20:00:00Z",
    "updatedAt": "2025-01-16T22:30:00Z",
    "id": 104,
    "expiry": "2025-01-30T08:00:00Z",
    "region": "OC",
    "refinement": "Intact",
    "activity": "Axi A4",
    "size": 4,
    "userId": 204
  },
  {
    "createdAt": "2025-01-15T07:30:00Z",
    "updatedAt": "2025-01-16T09:00:00Z",
    "id": 105,
    "expiry": "2025-02-05T20:00:00Z",
    "region": "SA",
    "refinement": "Flawless",
    "activity": "Axi A4",
    "size": 4,
    "userId": 205
  },
  {
    "createdAt": "2025-01-10T08:00:00Z",
    "updatedAt": "2025-01-12T10:30:00Z",
    "id": 101,
    "expiry": "2025-01-20T12:00:00Z",
    "region": "EU",
    "refinement": "Flawless",
    "activity": "Axi A4",
    "size": 4,
    "userId": 201
  },
  {
    "createdAt": "2025-01-11T09:15:00Z",
    "updatedAt": "2025-01-15T11:45:00Z",
    "id": 102,
    "expiry": "2025-02-01T23:59:59Z",
    "region": "NA",
    "refinement": "Intact",
    "activity": "Axi A4",
    "size": 4,
    "userId": 202
  },
  {
    "createdAt": "2025-01-13T14:45:00Z",
    "updatedAt": "2025-01-14T16:00:00Z",
    "id": 103,
    "expiry": "2025-01-25T18:00:00Z",
    "region": "AS",
    "refinement": "Radiant",
    "activity": "Axi A4",
    "size": 4,
    "userId": 203
  },
  {
    "createdAt": "2025-01-14T20:00:00Z",
    "updatedAt": "2025-01-16T22:30:00Z",
    "id": 104,
    "expiry": "2025-01-30T08:00:00Z",
    "region": "OC",
    "refinement": "Intact",
    "activity": "Axi A4",
    "size": 4,
    "userId": 204
  },
  {
    "createdAt": "2025-01-15T07:30:00Z",
    "updatedAt": "2025-01-16T09:00:00Z",
    "id": 105,
    "expiry": "2025-02-05T20:00:00Z",
    "region": "SA",
    "refinement": "Flawless",
    "activity": "Axi A4",
    "size": 4,
    "userId": 205
  }
]
`);
