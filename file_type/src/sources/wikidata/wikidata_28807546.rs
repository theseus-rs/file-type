use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28807546: FileFormat = FileFormat {
    id: 28_807_546,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Binder File for Windows 97-2000",
    extensions: &["obd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
