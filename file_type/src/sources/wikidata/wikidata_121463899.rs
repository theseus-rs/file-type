use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121463899: FileFormat = FileFormat {
    id: 121_463_899,
    source_type: SourceType::Wikidata,
    name: "Adobe Lightroom Database",
    extensions: &["lrdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
