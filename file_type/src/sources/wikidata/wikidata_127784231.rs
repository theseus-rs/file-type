use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127784231: FileFormat = FileFormat {
    id: 127_784_231,
    source_type: SourceType::Wikidata,
    name: "Logtalk source file",
    extensions: &["lgt"],
    media_types: &["text/x-logtalk"],
    signatures: &[],
    related_formats: &[],
};
