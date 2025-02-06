use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131341382: FileFormat = FileFormat {
    id: 131_341_382,
    source_type: SourceType::Wikidata,
    name: "Typst code",
    extensions: &["typ"],
    media_types: &["text/x-typst"],
    signatures: &[],
    related_formats: &[],
};
