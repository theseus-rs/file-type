use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48021588: FileFormat = FileFormat {
    id: 48_021_588,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Database, version 2000",
    extensions: &["mdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
