use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119217819: FileFormat = FileFormat {
    id: 119_217_819,
    source_type: SourceType::Wikidata,
    name: "QuickBooks Portable Company File",
    extensions: &["qbm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
