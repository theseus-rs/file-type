use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600263: FileFormat = FileFormat {
    id: 28_600_263,
    source_type: SourceType::Wikidata,
    name: "Ability Database",
    extensions: &["adb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
