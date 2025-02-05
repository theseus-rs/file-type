use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967349: FileFormat = FileFormat {
    id: 27_967_349,
    source_type: SourceType::Wikidata,
    name: "iTunes Music Library, binary variant",
    extensions: &["itl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
