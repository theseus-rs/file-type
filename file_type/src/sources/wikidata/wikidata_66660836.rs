use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66660836: FileFormat = FileFormat {
    id: 66_660_836,
    source_type: SourceType::Wikidata,
    name: "Adobe Premiere Sequence",
    extensions: &["psq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
