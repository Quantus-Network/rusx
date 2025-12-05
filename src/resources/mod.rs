use serde::{Deserialize, Serialize};

use crate::resources::{search::SearchMeta, user::User};

pub mod search;
pub mod tweet;
pub mod user;

pub trait AsQueryStr {
    fn as_str(&self) -> &'static str;
}

pub fn join_query_param_enums_as_string<T: AsQueryStr>(items: &[T]) -> String {
    items
        .iter()
        .map(|i| i.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserField {
    Affiliation,
    ConfirmedEmail,
    ConnectionStatus,
    CreatedAt,
    Description,
    Entities,
    Id,
    IsIdentityVerified,
    Location,
    MostRecentTweetId,
    Name,
    Parody,
    PinnedTweetId,
    ProfileBannerUrl,
    ProfileImageUrl,
    Protected,
    PublicMetrics,
    ReceivesYourDm,
    Subscription,
    SubscriptionType,
    Url,
    Username,
    Verified,
    VerifiedFollowersCount,
    VerifiedType,
    Withheld,
}

impl AsQueryStr for UserField {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Affiliation => "affiliation",
            Self::ConfirmedEmail => "confirmed_email",
            Self::ConnectionStatus => "connection_status",
            Self::CreatedAt => "created_at",
            Self::Description => "description",
            Self::Entities => "entities",
            Self::Id => "id",
            Self::IsIdentityVerified => "is_identity_verified",
            Self::Location => "location",
            Self::MostRecentTweetId => "most_recent_tweet_id",
            Self::Name => "name",
            Self::Parody => "parody",
            Self::PinnedTweetId => "pinned_tweet_id",
            Self::ProfileBannerUrl => "profile_banner_url",
            Self::ProfileImageUrl => "profile_image_url",
            Self::Protected => "protected",
            Self::PublicMetrics => "public_metrics",
            Self::ReceivesYourDm => "receives_your_dm",
            Self::Subscription => "subscription",
            Self::SubscriptionType => "subscription_type",
            Self::Url => "url",
            Self::Username => "username",
            Self::Verified => "verified",
            Self::VerifiedFollowersCount => "verified_followers_count",
            Self::VerifiedType => "verified_type",
            Self::Withheld => "withheld",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TweetField {
    Article,
    Attachments,
    AuthorId,
    CardUri,
    CommunityId,
    ContextAnnotations,
    ConversationId,
    CreatedAt,
    DisplayTextRange,
    EditControls,
    EditHistoryTweetIds,
    Entities,
    Geo,
    Id,
    InReplyToUserId,
    Lang,
    MediaMetadata,
    NonPublicMetrics,
    NoteTweet,
    OrganicMetrics,
    PossiblySensitive,
    PromotedMetrics,
    PublicMetrics,
    ReferencedTweets,
    ReplySettings,
    Scopes,
    Source,
    SuggestedSourceLinks,
    Text,
    Withheld,
}

impl AsQueryStr for TweetField {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Article => "article",
            Self::Attachments => "attachments",
            Self::AuthorId => "author_id",
            Self::CardUri => "card_uri",
            Self::CommunityId => "community_id",
            Self::ContextAnnotations => "context_annotations",
            Self::ConversationId => "conversation_id",
            Self::CreatedAt => "created_at",
            Self::DisplayTextRange => "display_text_range",
            Self::EditControls => "edit_controls",
            Self::EditHistoryTweetIds => "edit_history_tweet_ids",
            Self::Entities => "entities",
            Self::Geo => "geo",
            Self::Id => "id",
            Self::InReplyToUserId => "in_reply_to_user_id",
            Self::Lang => "lang",
            Self::MediaMetadata => "media_metadata",
            Self::NonPublicMetrics => "non_public_metrics",
            Self::NoteTweet => "note_tweet",
            Self::OrganicMetrics => "organic_metrics",
            Self::PossiblySensitive => "possibly_sensitive",
            Self::PromotedMetrics => "promoted_metrics",
            Self::PublicMetrics => "public_metrics",
            Self::ReferencedTweets => "referenced_tweets",
            Self::ReplySettings => "reply_settings",
            Self::Scopes => "scopes",
            Self::Source => "source",
            Self::SuggestedSourceLinks => "suggested_source_links",
            Self::Text => "text",
            Self::Withheld => "withheld",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TweetExpansion {
    #[serde(rename = "article.cover_media")]
    ArticleCoverMedia,

    #[serde(rename = "article.media_entities")]
    ArticleMediaEntities,

    #[serde(rename = "attachments.media_keys")]
    AttachmentsMediaKeys,

    #[serde(rename = "attachments.media_source_tweet")]
    AttachmentsMediaSourceTweet,

    #[serde(rename = "attachments.poll_ids")]
    AttachmentsPollIds,

    #[serde(rename = "author_id")]
    AuthorId,

    #[serde(rename = "edit_history_tweet_ids")]
    EditHistoryTweetIds,

    #[serde(rename = "entities.mentions.username")]
    EntitiesMentionsUsername,

    #[serde(rename = "geo.place_id")]
    GeoPlaceId,

    #[serde(rename = "in_reply_to_user_id")]
    InReplyToUserId,

    #[serde(rename = "entities.note.mentions.username")]
    EntitiesNoteMentionsUsername,

    #[serde(rename = "referenced_tweets.id")]
    ReferencedTweetsId,

    #[serde(rename = "referenced_tweets.id.attachments.media_keys")]
    ReferencedTweetsIdAttachmentsMediaKeys,

    #[serde(rename = "referenced_tweets.id.author_id")]
    ReferencedTweetsIdAuthorId,
}

impl AsQueryStr for TweetExpansion {
    fn as_str(&self) -> &'static str {
        match self {
            Self::ArticleCoverMedia => "article.cover_media",
            Self::ArticleMediaEntities => "article.media_entities",
            Self::AttachmentsMediaKeys => "attachments.media_keys",
            Self::AttachmentsMediaSourceTweet => "attachments.media_source_tweet",
            Self::AttachmentsPollIds => "attachments.poll_ids",
            Self::AuthorId => "author_id",
            Self::EditHistoryTweetIds => "edit_history_tweet_ids",
            Self::EntitiesMentionsUsername => "entities.mentions.username",
            Self::GeoPlaceId => "geo.place_id",
            Self::InReplyToUserId => "in_reply_to_user_id",
            Self::EntitiesNoteMentionsUsername => "entities.note.mentions.username",
            Self::ReferencedTweetsId => "referenced_tweets.id",
            Self::ReferencedTweetsIdAttachmentsMediaKeys => {
                "referenced_tweets.id.attachments.media_keys"
            }
            Self::ReferencedTweetsIdAuthorId => "referenced_tweets.id.author_id",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Includes {
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwitterApiResponse<T> {
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Includes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<SearchMeta>,
}
