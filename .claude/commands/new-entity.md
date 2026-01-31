Create a new domain entity with all required components.

Entity name: $ARGUMENTS

Follow the hexagonal architecture pattern to create:

## Domain Layer (`domain/`)
1. Entity struct in `domain/src/entity/{category}/`
2. Value objects in `domain/src/value_object/{category}/` (Id, Key, etc.)
   - Use newtype pattern: `pub struct Name(T)`
   - Implement `new()`, `value()`, `From`, `Display`
3. Repository trait in `domain/src/repository/{category}/`
4. Port trait in `domain/src/port/{category}/` (if external API needed)
5. Error types in `domain/src/error/`

## Infrastructure Layer (`infrastructure/`)
1. Repository implementation in `infrastructure/src/repository/{category}/`
2. API adapter in `infrastructure/src/adapter/{category}/` (if port defined)
3. DTOs with `TryFrom` implementations for domain conversion
4. Row structs in `infrastructure/src/database/` for SQLx
5. Database migration in `migrations/`

## Application Layer (`application/`)
1. UseCase trait and impl in `application/src/usecase/{category}/`
2. Error types in `application/src/error/{category}/`

## Presentation Layer (`presentation/`)
1. GraphQL types in `presentation/src/api/graphql/types/`
2. Query methods in `presentation/src/api/graphql/query/`
3. DataLoader in `presentation/src/api/graphql/dataloader/` (if needed)

Read `.claude/skills/clean-architecture/SKILL.md` for detailed patterns before implementing.

## Example File Structure for `{category}` Entity

```
domain/src/
├── entity/{category}/
│   ├── mod.rs
│   └── {entity_name}.rs
├── value_object/{category}/
│   ├── mod.rs
│   ├── {entity_name}_id.rs
│   └── {entity_name}_key.rs
├── repository/{category}/
│   ├── mod.rs
│   └── {entity_name}_repository.rs
├── port/{category}/
│   ├── mod.rs
│   └── {entity_name}_port.rs
└── error/
    └── {entity_name}_error.rs
```
