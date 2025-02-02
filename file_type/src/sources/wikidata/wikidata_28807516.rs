use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28807516: FileFormat = FileFormat {
    id: 28_807_516,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Binder File for Windows 95",
    extensions: &["obd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
