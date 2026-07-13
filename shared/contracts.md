# Shared Contracts

The frontend and backend share these stable concepts:

```json
{
  "playerTag": "#ABC123",
  "coachRequest": {
    "scope": ["heroes", "troops", "equipment", "buildings"],
    "aiProvider": "openrouter|ollama"
  },
  "recommendation": {
    "target": "Archer Queen",
    "priority": 95,
    "reason": "High attack impact",
    "estimatedImpact": "High"
  }
}
```
