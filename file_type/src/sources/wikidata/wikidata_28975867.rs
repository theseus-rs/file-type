use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975867: FileFormat = FileFormat {
    id: 28_975_867,
    source_type: SourceType::Wikidata,
    name: "OOGL SKEL file",
    extensions: &["skel"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
