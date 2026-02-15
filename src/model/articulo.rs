use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Articulo {
    pub bloque_id: String,           
    pub articulo_numero: String,     
    pub id_norma: String,            
    pub fecha_publicacion: NaiveDate,
    pub fecha_vigencia: NaiveDate,
}
