use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_82025103: FileFormat = FileFormat {
    id: 82_025_103,
    source_type: SourceType::Wikidata,
    name: "Microsoft Reader eBook annotations",
    extensions: &["ebo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
