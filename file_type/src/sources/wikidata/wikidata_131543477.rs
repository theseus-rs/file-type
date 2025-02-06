use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131543477: FileFormat = FileFormat {
    id: 131_543_477,
    source_type: SourceType::Wikidata,
    name: "Varian FDF file format",
    extensions: &["fdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
