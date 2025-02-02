use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48004607: FileFormat = FileFormat {
    id: 48_004_607,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Database, version 2",
    extensions: &["mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
