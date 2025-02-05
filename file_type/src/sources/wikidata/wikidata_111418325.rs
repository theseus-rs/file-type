use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111418325: FileFormat = FileFormat {
    id: 111_418_325,
    source_type: SourceType::Wikidata,
    name: "Adobe Bridge Workspace File",
    extensions: &["workspace"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
