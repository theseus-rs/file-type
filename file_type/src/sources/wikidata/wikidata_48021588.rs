use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48021588: FileFormat = FileFormat {
    id: 48_021_588,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Database, version 2000",
    extensions: &["mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
