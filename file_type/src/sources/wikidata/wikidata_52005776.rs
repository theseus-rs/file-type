use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52005776: FileFormat = FileFormat {
    id: 52_005_776,
    source_type: SourceType::Wikidata,
    name: "Hewlett Packard Graphics Language format",
    extensions: &["hpgl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
