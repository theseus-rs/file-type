use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48005022: FileFormat = FileFormat {
    id: 48_005_022,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Database, version 97",
    extensions: &["mdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
