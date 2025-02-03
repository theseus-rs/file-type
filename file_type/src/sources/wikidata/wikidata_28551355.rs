use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28551355: FileFormat = FileFormat {
    id: 28_551_355,
    source_type: SourceType::Wikidata,
    name: "Adobe Hue/Saturation File",
    extensions: &["ahu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
