use zenith_stellar-co-star-zenith-stellar-co-star-zenith-backend::db::{detect_stellar-co-star-zenith-stellar-co-star-zenith-backend, DatabaseBackend};

#[test]
fn test_database_scheme_detection() {
    assert_eq!(
        detect_stellar-co-star-zenith-stellar-co-star-zenith-backend("postgres://user:pass@localhost:5432/zenith"),
        DatabaseBackend::Postgres
    );
    assert_eq!(
        detect_stellar-co-star-zenith-stellar-co-star-zenith-backend("postgresql://user:pass@localhost:5432/zenith"),
        DatabaseBackend::Postgres
    );
    assert_eq!(
        detect_stellar-co-star-zenith-stellar-co-star-zenith-backend("sqlite://zenith.db"),
        DatabaseBackend::Sqlite
    );
    assert_eq!(
        detect_stellar-co-star-zenith-stellar-co-star-zenith-backend("sqlite::memory:"),
        DatabaseBackend::Sqlite
    );
}
