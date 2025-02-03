use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131341382: FileFormat = FileFormat {
    id: 131_341_382,
    source_type: SourceType::Wikidata,
    name: "Typst code",
    extensions: &["typ"],
    media_types: &["text/x-typst"],
    internal_signatures: &[],
    related_formats: &[],
};
