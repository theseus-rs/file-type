use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48782394: FileFormat = FileFormat {
    id: 48_782_394,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word for MS-DOS Document, version 5.5",
    extensions: &["doc"],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
