use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205933: FileFormat = FileFormat {
    id: 28_205_933,
    source_type: SourceType::Wikidata,
    name: "Doodle! uncompressed image",
    extensions: &["dd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
