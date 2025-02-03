use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48021763: FileFormat = FileFormat {
    id: 48_021_763,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Database, version 2002",
    extensions: &["mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
