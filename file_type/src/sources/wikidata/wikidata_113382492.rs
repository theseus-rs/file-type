use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113382492: FileFormat = FileFormat {
    id: 113_382_492,
    source_type: SourceType::Wikidata,
    name: "Roxio Easy Media Creator Classic Creator File 8-10",
    extensions: &["rcl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
