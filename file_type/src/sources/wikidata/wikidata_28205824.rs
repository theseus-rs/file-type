use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205824: FileFormat = FileFormat {
    id: 28_205_824,
    source_type: SourceType::Wikidata,
    name: "CgBI",
    extensions: &["png"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
