use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167468: FileFormat = FileFormat {
    id: 29_167_468,
    source_type: SourceType::Wikidata,
    name: "Open Virtualization Format Archive Package",
    extensions: &["ova"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
