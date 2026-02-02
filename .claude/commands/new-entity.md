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

## Application Layer (`application/`) - CQRS Pattern
1. Command UseCase in `application/src/usecase/command/{category}/`
   - Sync, Create, Update use cases (write operations)
2. Query UseCase in `application/src/usecase/query/{category}/`
   - FindByIds, List use cases (read operations, returns DTOs)
3. Query Repository trait in `application/src/repository/{category}/`
4. Query DTOs in `application/src/dto/query/{category}/`
5. Command DTOs in `application/src/dto/command/{category}/` (if needed)
6. Command errors in `application/src/error/command/{category}/`
7. Query errors in `application/src/error/query/{category}/`

## Infrastructure Layer (`infrastructure/`)
1. Command Repository implementation in `infrastructure/src/repository/command/{category}/`
2. Query Repository implementation in `infrastructure/src/repository/query/{category}/`
3. API adapter in `infrastructure/src/adapter/{category}/` (if port defined)
4. External API DTOs in `infrastructure/src/dto/{category}/`
5. Row structs in `infrastructure/src/database/` for SQLx
   - Implement `into_dto()` for query operations
   - Implement `from_domain()` for command operations
6. Database migration in `migrations/`

## Presentation Layer (`presentation/`)
1. GraphQL types in `presentation/src/api/graphql/types/` (`{Entity}Gql`, `{Entity}ListGql`)
2. Query methods in `presentation/src/api/graphql/query/`
3. Mutation methods in `presentation/src/api/graphql/mutation/` (if write operations needed)
4. Input types for mutations (`{Action}{Entity}InputGql`)
5. DataLoader in `presentation/src/api/graphql/dataloader/` (for N+1 prevention)

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
├── repository/{category}/           # Command repository trait
│   ├── mod.rs
│   └── {entity_name}_repository.rs
├── port/{category}/
│   ├── mod.rs
│   └── {entity_name}_port.rs
└── error/
    └── {entity_name}_error.rs

application/src/
├── usecase/
│   ├── command/{category}/
│   │   ├── mod.rs
│   │   ├── {entity_name}_sync_usecase.rs
│   │   ├── {entity_name}_create_usecase.rs
│   │   └── {entity_name}_update_usecase.rs
│   └── query/{category}/
│       ├── mod.rs
│       ├── {entity_name}_find_by_ids_query_usecase.rs
│       └── {entity_name}_list_query_usecase.rs
├── repository/{category}/           # Query repository trait
│   ├── mod.rs
│   └── {entity_name}_query_repository.rs
├── dto/
│   ├── command/{category}/
│   │   ├── mod.rs
│   │   ├── create_{entity_name}_dto.rs
│   │   └── update_{entity_name}_dto.rs
│   └── query/{category}/
│       ├── mod.rs
│       └── {entity_name}_query_dto.rs
└── error/
    ├── command/{category}/
    └── query/{category}/

infrastructure/src/
├── repository/
│   ├── command/{category}/
│   │   └── {entity_name}_repository_impl.rs
│   └── query/{category}/
│       └── {entity_name}_query_repository_impl.rs
├── adapter/{category}/
│   └── {entity_name}_adapter_impl.rs
├── dto/{category}/
│   └── {entity_name}_dto.rs
└── database/
    └── {entity_name}_row.rs

presentation/src/api/graphql/
├── types/
│   ├── {entity_name}.rs           # {Entity}Gql
│   ├── {entity_name}_list.rs      # {Entity}ListGql
│   └── {entity_name}_input.rs     # Create/Update input
├── query/
│   └── {entity_name}_query.rs
├── mutation/
│   └── {entity_name}_mutation.rs
└── dataloader/
    └── {entity_name}_loader.rs
```
