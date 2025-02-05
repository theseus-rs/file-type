use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66689263: FileFormat = FileFormat {
    id: 66_689_263,
    source_type: SourceType::Wikidata,
    name: "Access (SQL Server) detached database",
    extensions: &["mdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
