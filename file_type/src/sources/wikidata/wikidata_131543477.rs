use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131543477: FileFormat = FileFormat {
    id: 131_543_477,
    source_type: SourceType::Wikidata,
    name: "Varian FDF file format",
    extensions: &["fdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
