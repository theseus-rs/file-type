use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129167658: FileFormat = FileFormat {
    id: 129_167_658,
    source_type: SourceType::Wikidata,
    name: "Ezhil file format",
    extensions: &["n"],
    media_types: &["text/x-ezhil"],
    signatures: &[],
    related_formats: &[],
};
