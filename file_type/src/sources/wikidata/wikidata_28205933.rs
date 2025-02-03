use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205933: FileFormat = FileFormat {
    id: 28_205_933,
    source_type: SourceType::Wikidata,
    name: "Doodle! uncompressed image",
    extensions: &["dd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
