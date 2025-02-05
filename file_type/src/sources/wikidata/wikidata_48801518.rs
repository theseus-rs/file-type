use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48801518: FileFormat = FileFormat {
    id: 48_801_518,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 8",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    signatures: &[],
    related_formats: &[],
};
