use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129167658: FileFormat = FileFormat {
    id: 129_167_658,
    source_type: SourceType::Wikidata,
    name: "Ezhil file format",
    extensions: &["n"],
    media_types: &["text/x-ezhil"],
    internal_signatures: &[],
    related_formats: &[],
};
