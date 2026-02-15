use sqlx::{PgPool, postgres::PgPoolOptions, query};
use anyhow::Result;
use crate::model::articulo::Articulo;

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn upsert_articulo(pool: &PgPool, articulo: &Articulo) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO articulos (bloque_id, articulo_numero, id_norma, fecha_publicacion, fecha_vigencia)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (bloque_id) DO UPDATE
        SET articulo_numero = EXCLUDED.articulo_numero,
            id_norma = EXCLUDED.id_norma,
            fecha_publicacion = EXCLUDED.fecha_publicacion,
            fecha_vigencia = EXCLUDED.fecha_vigencia
        "#,
        articulo.bloque_id,
        articulo.articulo_numero,
        articulo.id_norma,
        articulo.fecha_publicacion,
        articulo.fecha_vigencia
    )
    .execute(pool)
    .await?;

    Ok(())
}
