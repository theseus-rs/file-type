use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_96054590: FileFormat = FileFormat {
    id: 96_054_590,
    source_type: SourceType::Wikidata,
    name: "Macromolecular Crystallographic Information File",
    extensions: &["mmcif"],
    media_types: &["chemical/x-mmcif"],
    signatures: &[],
    related_formats: &[],
};
