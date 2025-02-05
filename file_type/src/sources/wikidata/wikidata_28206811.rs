use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206811: FileFormat = FileFormat {
    id: 28_206_811,
    source_type: SourceType::Wikidata,
    name: "Paint.NET image",
    extensions: &["pdn"],
    media_types: &["image/x-paintnet"],
    signatures: &[],
    related_formats: &[],
};
