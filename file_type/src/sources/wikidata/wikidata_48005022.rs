use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48005022: FileFormat = FileFormat {
    id: 48_005_022,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Database, version 97",
    extensions: &["mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
