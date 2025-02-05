use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111363669: FileFormat = FileFormat {
    id: 111_363_669,
    source_type: SourceType::Wikidata,
    name: "Wusikstation V3 sound file",
    extensions: &["wusiksnd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
