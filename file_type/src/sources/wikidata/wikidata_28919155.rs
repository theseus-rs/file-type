use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919155: FileFormat = FileFormat {
    id: 28_919_155,
    source_type: SourceType::Wikidata,
    name: "Rhino Worksession",
    extensions: &["rws"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
