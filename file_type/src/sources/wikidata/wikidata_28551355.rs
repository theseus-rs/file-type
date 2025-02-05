use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28551355: FileFormat = FileFormat {
    id: 28_551_355,
    source_type: SourceType::Wikidata,
    name: "Adobe Hue/Saturation File",
    extensions: &["ahu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
