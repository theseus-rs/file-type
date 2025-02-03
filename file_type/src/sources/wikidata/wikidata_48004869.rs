use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48004869: FileFormat = FileFormat {
    id: 48_004_869,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Database, version 95",
    extensions: &["mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
