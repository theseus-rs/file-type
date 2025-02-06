use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63391433: FileFormat = FileFormat {
    id: 63_391_433,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for DOS",
    extensions: &["wdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
