use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48004869: FileFormat = FileFormat {
    id: 48_004_869,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Database, version 95",
    extensions: &["mdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
