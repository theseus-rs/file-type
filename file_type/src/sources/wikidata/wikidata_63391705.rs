use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63391705: FileFormat = FileFormat {
    id: 63_391_705,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for DOS, version 3b",
    extensions: &["wdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
