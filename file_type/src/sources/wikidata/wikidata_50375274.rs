use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50375274: FileFormat = FileFormat {
    id: 50_375_274,
    source_type: SourceType::Wikidata,
    name: "Microsoft OneNote Package File",
    extensions: &["onepkg"],
    media_types: &["application/onenote"],
    internal_signatures: &[],
    related_formats: &[],
};
