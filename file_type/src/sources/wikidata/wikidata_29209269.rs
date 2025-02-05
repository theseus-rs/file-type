use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29209269: FileFormat = FileFormat {
    id: 29_209_269,
    source_type: SourceType::Wikidata,
    name: "Z",
    extensions: &["z"],
    media_types: &["application/x-compress"],
    signatures: &[],
    related_formats: &[],
};
