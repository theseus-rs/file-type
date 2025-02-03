use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27863098: FileFormat = FileFormat {
    id: 27_863_098,
    source_type: SourceType::Wikidata,
    name: "3GPP2 file format",
    extensions: &["3g2"],
    media_types: &["audio/3gpp2", "video/3gpp2"],
    internal_signatures: &[],
    related_formats: &[],
};
