use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857392: FileFormat = FileFormat {
    id: 105_857_392,
    source_type: SourceType::Wikidata,
    name: "Java Web Start application descriptor",
    extensions: &["jnlp"],
    media_types: &["text.xnk"],
    signatures: &[],
    related_formats: &[],
};
