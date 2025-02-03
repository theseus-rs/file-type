use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206811: FileFormat = FileFormat {
    id: 28_206_811,
    source_type: SourceType::Wikidata,
    name: "Paint.NET image",
    extensions: &["pdn"],
    media_types: &["image/x-paintnet"],
    internal_signatures: &[],
    related_formats: &[],
};
